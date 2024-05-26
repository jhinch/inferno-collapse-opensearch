use std::collections::VecDeque;
use std::io::{BufRead, Write};
use inferno::collapse::Collapse;
use crate::profile::*;

#[derive(Default)]
pub struct Folder {
    stack: VecDeque<String>,
}

impl Folder {
    fn write_line<W>(&self, count: usize, writer: &mut W) -> std::io::Result<()> where W: Write {
        let mut sep = "";
        for item in self.stack.iter() {
            write!(writer, "{}{}", sep, item)?;
            sep = ";";
        }
        writeln!(writer, " {}", count)
    }

    fn collapse_query<W>(&mut self, query: &ProfileQuery, key: &str, writer: &mut W) -> std::io::Result<()> where W: Write {
        self.stack.push_back(safe_segment(&format!("{}({}).{}", query.ty, query.description, key)));
        self.write_line(query.breakdown[key], writer)?;
        for child in &query.children {
            self.collapse_query(child, key, writer)?;
        }
        self.stack.pop_back();
        Ok(())
    }
}

fn safe_segment(s: &str) -> String {
    s.replace(" ", "_").replace(";", ":")
}

fn non_inclusive_times(profile: &mut Profile) {
    for shard in profile.shards.iter_mut() {
        for search in shard.searches.iter_mut() {
            for query in search.query.iter_mut() {
                let keys: Vec<String> = query.breakdown.keys().cloned().collect();
                let mut exclude_nanos = 0;
                for key in keys {
                    exclude_nanos += non_inclusive_times_for_key(query, &key)
                }
                query.time_in_nanos = query.time_in_nanos.saturating_sub(exclude_nanos);
            }
        }
    }
}

fn non_inclusive_times_for_key(query: &mut ProfileQuery, key: &str) -> usize {
    let inclusive_nanos = query.breakdown[key];
    let mut exclude_nanos = 0;
    for child in query.children.iter_mut() {
        exclude_nanos += non_inclusive_times_for_key(child, key)
    }
    *query.breakdown.get_mut(key).unwrap() = inclusive_nanos.saturating_sub(exclude_nanos);
    inclusive_nanos
}

impl Collapse for Folder {
    fn collapse<R, W>(&mut self, reader: R, mut writer: W) -> std::io::Result<()> where R: BufRead, W: Write {
        let mut profile: Profile = serde_json::from_reader(reader)?;
        non_inclusive_times(&mut profile);
        for shard in &profile.shards {
            self.stack.push_back(safe_segment(&shard.id));
            self.write_line(0, &mut writer)?;
            for search in &shard.searches {
                self.stack.push_back(safe_segment("search"));
                self.write_line(search.rewrite_time, &mut writer)?;
                for query in &search.query {
                    self.stack.push_back(safe_segment(&format!("{}({})", query.ty, query.description)));
                    self.write_line(query.time_in_nanos, &mut writer)?;
                    for key in query.breakdown.keys().cloned().collect::<Vec<_>>() {
                        self.collapse_query(query, &key, &mut writer)?;
                    }
                    self.stack.pop_back();
                }
                self.stack.pop_back();
            }
            self.stack.pop_back();
        }
        Ok(())
    }

    fn is_applicable(&mut self, input: &str) -> Option<bool> {
        Some(true)
    }
}