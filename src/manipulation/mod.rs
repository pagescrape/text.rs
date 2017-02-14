use regex::Regex;
use std::borrow::Cow;

pub trait Manipulation {
    fn average_length(&self) -> f64;
    fn char_count(&self) -> usize;
    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>>;
    fn punct_count(&self) -> usize;
    fn words_count(&self) -> usize;
    fn words(&self) -> Vec<&str>;
}

impl<'s> Manipulation for Vec<Cow<'s, str>> {
    fn char_count(&self) -> usize {
        self.iter().fold(0, |sum, i| sum + i.chars().count())
    }

    fn words_count(&self) -> usize {
        self.len()
    }

    fn average_length(&self) -> f64 {
        let total = self.char_count();
        if total == 0 {
            return 0.0;
        }
        total as f64 / self.len() as f64
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        self.iter()
            .map(|s| punct.replace_all(s, ""))
            .filter(|f| !f.is_empty())
            .collect()
    }

    fn punct_count(&self) -> usize {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        self.iter().fold(0, |sum, text| sum + punct.find_iter(text).count())
    }

    fn words(&self) -> Vec<&str> {
        self.iter().flat_map(|s| s.split_whitespace()).collect()
    }
}

impl<'s> Manipulation for Vec<&'s str> {
    fn char_count(&self) -> usize {
        self.iter().fold(0, |sum, i| sum + i.chars().count())
    }

    fn words_count(&self) -> usize {
        self.len()
    }

    fn average_length(&self) -> f64 {
        let total = self.char_count();
        if total == 0 {
            return 0.0;
        }
        total as f64 / self.words_count() as f64
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        self.iter()
            .map(|s| punct.replace_all(s, ""))
            .filter(|f| !f.is_empty())
            .collect()
    }

    fn punct_count(&self) -> usize {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        self.iter().fold(0, |sum, text| sum + punct.find_iter(text).count())
    }

    fn words(&self) -> Vec<&str> {
        self.iter().flat_map(|s| s.split_whitespace()).collect()
    }
}

impl Manipulation for String {
    fn average_length(&self) -> f64 {
        self.len() as f64
    }

    fn char_count(&self) -> usize {
        self.chars().count()
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        vec![punct.replace_all(self, "")]
    }

    fn punct_count(&self) -> usize {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        punct.find_iter(self).count()
    }

    fn words_count(&self) -> usize {
        if self.len() == 0 { 0 } else { 1 }
    }

    fn words(&self) -> Vec<&str> {
        self.split_whitespace().collect()
    }
}

impl Manipulation for Vec<String> {
    fn words_count(&self) -> usize {
        self.len()
    }

    fn char_count(&self) -> usize {
        self.iter().fold(0, |sum, i| sum + i.chars().count())
    }

    fn average_length(&self) -> f64 {
        let total = self.iter().fold(0, |sum, i| sum + i.chars().count());
        if total == 0 {
            return 0.0;
        }
        total as f64 / self.len() as f64
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        self.iter()
            .map(|s| punct.replace_all(s, ""))
            .filter(|f| !f.is_empty())
            .collect()
    }

    fn punct_count(&self) -> usize {
        let punct: Regex = Regex::new("[[:punct:]]").unwrap();
        self.iter().fold(0, |sum, text| sum + punct.find_iter(text).count())
    }

    fn words(&self) -> Vec<&str> {
        self.iter().flat_map(|s| s.split_whitespace()).collect()
    }
}