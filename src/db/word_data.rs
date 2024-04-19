#[derive(Debug)]
pub struct WordData {
    pub id: i32,
    pub word: String,
    pub sw: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub pos: String,
    pub collins: i32,
    pub oxford: i32,
    pub tag: String,
    pub bnc: i32,
    pub frq: i32,
    pub exchange: String,
    pub detail: String,
    pub audio: String,
}
