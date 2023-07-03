# musixmatch-rs

[![Crates.io](https://img.shields.io/crates/v/my_crate.svg)](https://crates.io/crates/musixmatch)
[![Docs.rs](https://docs.rs/my_crate/badge.svg)](https://docs.rs/musixmatch)
[![License](https://img.shields.io/crates/l/musixmatch.svg)](https://github.com//Deaths-Door/musixmatch-rs/blob/main/LICENSE)

This repository offers a wide range of functionalities that can be leveraged to build music applications requiring access to this musicmatch. By utilizing this Rust implementation, developers can seamlessly integrate MusixMatch API capabilities into their Rust projects, enabling the development of powerful and feature-rich music applications.

## About

The MusixMatch API is a comprehensive service that provides access to music metadata, lyrics, and album art. It offers a wide range of functionalities, including:

- Searching for songs, artists, and albums
- Retrieving lyrics for a given song
- Accessing album details and artwork
- Discovering top tracks, artists, and albums by country
- Fetching music genres and related data

By leveraging the MusixMatch API, developers can build music applications that require access to this rich set of music-related data. The `musixmatch-rs` crate provides a convenient Rust implementation for interacting with the MusixMatch API, enabling seamless integration into Rust projects.

For more information about the MusixMatch API and its capabilities, please refer to the [MusixMatch API Documentation](https://developer.musixmatch.com/documentation).

## Features

- `marocs`: Enables the use of default arguments for methods using the `default-args` crate.

## Installation

Add this to your `Cargo.toml` for default features:

```toml
[dependencies]
musixmatch = "0.1.0"
```

With the `marocs` feature enabled, you can use default arguments for methods in the MusixAbgleich API. This allows you to omit certain parameters and have them set to default values automatically (None).

```toml
[dependencies]
musixmatch = { version = "0.1.0", features = ["marcos"] }
```

## Usage

To use the MusixMatch API, you need an API key. Follow these steps to obtain an API key:

- Visit the [MusixMatch Developer Portal](https://developer.musixmatch.com/) and click on "Getting Started" to begin the process.
- Fill in the required information to create a developer account. Provide your name, email address, and choose a password. Accept the terms and conditions, and then click on "Sign Up" to create your account.
- Click on "Create New Application" to start creating a new application. Provide the necessary details such as the application name, description, and website URL. You may also need to specify the type of application and agree to any additional terms or policies.
- Once you have filled in the required information, submit the application.
- After your application is reviewed and approved, you will receive an email notification containing your API key. This key will be associated with the application you created.

`Note` : The exact steps and process may vary slightly depending on any updates or changes made to the MusixMatch Developer Portal. Please refer to the official documentation provided by MusixMatch for the most up-to-date instructions on obtaining an API key.

```rust
use musixmatch::MusixAbgleich;

fn main() {
    // Create an instance of MusixAbgleich
    let musicabgleich = MusixAbgleich::new("your_api_key",||{
        // Custom error handler for handling status codes 
        // Look below for possible status codes from music match
    });

    // Call methods with default arguments
    let artists = musicabgleich.top_artists_by_country(Some("US"), None, None);
    println!("{:?}", artists);

    //Or when using marcos feature
    let marco_feature_artist = top_artists_by_country!(musicabgleich,country = "US");
    println!("{:?}", marco_feature_artist);
}
```

Please note that the examples provided here are simplified and serve as a starting point. For comprehensive documentation of the crate, please visit the [crate documentation](https://docs.rs/musixmatch) for a better understanding of the crate's functionalities and APIs.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
