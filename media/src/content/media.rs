// Derive Debug trait to print enum values
#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn print(&self) {
        println!("{:#?}", self);
    }

    pub fn description(&self) -> String {
        // if let Media::Book {title, author} = self {
        //     return format!("{} by {}", title, author);
        // }
        // if let Media::Movie {title, director} = self {
        //     return format!("{} directed by {}", title, director);
        // }
        // if let Media::AudioBook {title} = self {
        //     return format!("{}", title);
        // }
        match self {
            Media::Book { title, author } => {
                format!("{} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("{} directed by {}", title, director)
            }
            Media::AudioBook { title } => format!("{}", title),
            Media::Podcast(episode) => format!("Episode {}", episode),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}
