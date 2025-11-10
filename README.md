A multi-agent simulation in Rust, with GIS integration.

About
-----
This was created to model the interactions of mobile populations with cell towers. During a previous role, testing methods for locations was an issue as creating synthetic data of anything approaching the correct shape was not strightforward. This crate aims to address that issue for anyone that needs it; if you're developing methods based on CRD data, I'm hoping this will be of use to you.

What it does
------------
Currently loads a .geojson of areas, populates each with a scattering of random cell towers and a set of pheeples.
Each pheeple has a home location and a work location, randomly allocated within their area.
A day takes 5 seconds by default; every morning, the pheeples commute to work and every evening they commute home.
Each pheeple has a chance-per-tick to make a call; they are connected to a random cell tower weighted to their proximity.
The final output of the program is a .csv containing timestamped records of calls made by pheeple, and what tower that was via.

Installing and running
----------------------
You'll need [Cargo]
Currently, you need to clone this repo and run `cargo run --release -- --basemap path/to/a/geojson`; this will pop open a window showing your simulation. Leave it running for a bit, and then find your mock data at `outputs/call_records.csv`.

Future
------
This is a personal project, mainly as a Rust and Bevy exercise. That said, I've got a small list of improvements and features I want to work on:
 
- Docs
- Better distribution via Cargo crate and/or build artefacts.
- Complete rework of `gis.rs` to use [https://github.com/aevyrie/big_space](https://github.com/aevyrie/big_space) and some other GIS crates
  - Once done, implement scrolling and zoomable camera
  - ~Also make the populations limited to their areas (they're just a crude bounding box right now)~ Done
- Loading population densities and other metadata from the geojson, instead of just randomly populating areas
- Likewise for loading arbitrary cell tower maps
- Advanced Pheeple behavior:
  - Differing likelihood of phone use to reflect phone ownership patterns
  - Weekends
  - Travel via roads instead of direcly to their destination
- Advanced 'scenario' design
  - 'simulation time' instead of 'seconds passed'
  - Fleeing and displacements due to disasters
  - Tower outages
  - A timeline format for putting together chains of events that happen at certain dates
- Headless mode for running larger simulations

[Screencast from 23-10-25 17:37:13.webm](https://github.com/user-attachments/assets/70016966-b05d-4577-93f6-e5a47df53407)

The name
--------
It's 'phone meeples', singlular 'pheeple', plural 'pheeples'. This is a design decision I have quickly come to regret.
They were originally going to be called 'phonies', but that didn't seem right somehow.
