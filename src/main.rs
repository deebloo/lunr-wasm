mod index;
mod query;
mod util;

use index::Index;
use std::fs;

fn main() {
    let mut index = Index::new();

    index.add_document("0", "foo bar Cyclone Seroja (pictured) makes landfall in Indonesia and East Timor, killing at least 113 people and displacing thousands of others.");
    index.add_document("1", "foo The Statute of Anne, the first legislation in Great Britain providing for copyright regulated by the government and courts, received royal assent and went into effect five days later.");
    index.add_document("2", "foo bar Eunice Pinney (1770–1849) was an American folk artist active in the towns of Windsor and Simsbury, Connecticut. According to art historian Jean Lipman, a specialist in American folk painting, Pinney and her contemporary Mary Ann Willson are considered two of the earliest American painters to work in the medium of watercolor.");

    let index_data = index.export_index();

    fs::write("www/search_index.bin", index_data).unwrap();

    let res = index.search("foo bar");

    println!("{:?}", res);
}
