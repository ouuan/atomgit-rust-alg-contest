use dsu::DisjointSetUnion;
use serde::de::{Deserialize, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use std::collections::HashMap;
use std::fmt;

pub fn count_provinces() -> String {
    const INPUT_STR: &str = include_str!("../district.json");
    serde_json::from_str::<Answer>(INPUT_STR)
        .expect("invalid input")
        .0
}

/// Solve a single testcase via disjoint set union.
#[derive(Default)]
struct Solver<'a> {
    id_map: HashMap<&'a str, usize>,
    dsu: DisjointSetUnion,
}

impl<'a> Solver<'a> {
    fn get_or_create_id(&mut self, city: &'a str) -> usize {
        *self
            .id_map
            .entry(city)
            .or_insert_with(|| self.dsu.new_element())
    }
}

/// Add an array of relations to solver during deserialization.
struct RelationArrayConsumer<'a> {
    from: usize,
    solver: Solver<'a>,
}

impl<'de> DeserializeSeed<'de> for RelationArrayConsumer<'de> {
    type Value = Solver<'de>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_seq(self)
    }
}

impl<'de> Visitor<'de> for RelationArrayConsumer<'de> {
    type Value = Solver<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("relation array")
    }

    fn visit_seq<A: SeqAccess<'de>>(mut self, mut seq: A) -> Result<Self::Value, A::Error> {
        while let Some(city) = seq.next_element()? {
            let v = self.solver.get_or_create_id(city);
            self.solver.dsu.union(self.from, v);
        }
        Ok(self.solver)
    }
}

struct TestCaseAnswer(usize);

impl<'de> Deserialize<'de> for TestCaseAnswer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(TestCaseVisitor)
    }
}

/// Solve a testcase during deserialization.
struct TestCaseVisitor;

impl<'de> Visitor<'de> for TestCaseVisitor {
    type Value = TestCaseAnswer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("testcase")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut solver = Solver::default();
        while let Some(key) = map.next_key()? {
            let from = solver.get_or_create_id(key);
            solver = map.next_value_seed(RelationArrayConsumer { from, solver })?;
        }
        Ok(TestCaseAnswer(solver.dsu.count_disjoint()))
    }
}

struct Answer(String);

impl<'de> Deserialize<'de> for Answer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(InputVisitor)
    }
}

/// Gather answers to all testcases during deserialization.
struct InputVisitor;

impl<'de> Visitor<'de> for InputVisitor {
    type Value = Answer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("input")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut answers = Vec::new();
        while let Some((_, value)) = map.next_entry::<&str, TestCaseAnswer>()? {
            answers.push(value.0.to_string());
        }
        Ok(Answer(answers.join(",")))
    }
}
