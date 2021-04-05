mod index;

use index::Index;

fn main() {
    let mut index = Index::new();

    index.add_document("0".to_string(), String::from("Christ lag in Todes Banden (Christ lay in death's bonds), BWV 4, is a chorale cantata for Easter by Johann Sebastian Bach, one of his earliest church cantatas. It is agreed to be an early work, partly for stylistic reasons and partly because there is evidence that it was probably written for a performance in 1707. Text and music are based on Luther's hymn of the same name, derived from medieval models. In each of seven vocal movements, Bach used the unchanged words of a stanza of the chorale and its tune as a cantus firmus. Although all movements are in E minor, Bach intensified the meaning of the text through a variety of musical forms and techniques. He performed the cantata again as Thomaskantor in Leipzig, beginning in 1724 for his first Easter there. Only this second version survived, scored for four vocal parts (soprano part pictured) and a Baroque instrumental ensemble with strings and a choir of cornetto and three trombones. John Eliot Gardiner described the cantata as Bach's \"first-known attempt at painting narrative in music\" and \"a bold, innovative piece of musical drama\". "));

    let res = index.search("Christ".to_string());

    println!("{:#?}", res);
}
