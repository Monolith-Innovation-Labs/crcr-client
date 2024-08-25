# Monolith Lab's CRCR
A minimal CRCR client written in Rust.

***Note: This project is still under development and is not yet ready for use.***

## Why Another CRCR Client?
CRCR is a mod for S.T.A.L.K.E.R. that adds MMO-like chat functionality to the game, using IRC as the backend. Due to the limitations of the X-Ray engine, the game cannot send HTTP requests directly. To work around this, CRCR writes chat messages to a file, then an external application (crcr.exe) is then used to read these messages from the file and send them to an IRC server.

The current official app, however, is written in an outdated version of .NET from 9 years ago and also acts as a full IRC client, displaying messages, allowing chat interaction and changing settings.

I believe this external app should focus solely on the task of reading messages from the file and transmitting them to the IRC server, without displaying any messages or providing additional chat features. All interactions and settings should be managed directly within the game to maintain immersion, simplify the setup, and speedup the development process.

## Will This Split the Community?
Not at all. This client will use the same IRC channels as AnchorPoint CRCR, ensuring the community remains connected.

## How to install?
1. Download the binary
2. Unpack the archive into your game directory
3. Run `crcr.exe`
4. Launch the game

## Using Mod Organizer?
If you're using Mod Organizer, add `crcr.exe` as an executable and run it through Mod Organizer before launching Anomaly.
