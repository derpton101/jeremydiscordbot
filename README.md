# Discord Anti-Phishing & Antivirus Bot

## Overview

This bot is designed to provide an additional layer of security for Discord servers by detecting and blocking phishing attempts, particularly masked links, and by integrating antivirus scanning. It also features the ability to automatically delete messages from specified users, helping moderators control disruptive behavior.

The project is a work in progress and was developed solely by me over random intervals. While the core functionality is in place, there is still room for improvement and further feature additions.

## Features

* **Anti-Phishing:** Detects masked links in the `[text](link)` Markdown format using regex to prevent common phishing tactics.
* **Antivirus Integration:** Uses ClamAV to scan message contents for known malware signatures.
* **Message Deletion:** Automatically deletes any messages sent by targeted users within the server to assist moderation.
* **Extensible:** Planned addition of link detection through PhishTank API for enhanced protection.
* **Self-Hosting:** Currently runs locally, but requires a permanent server to stay online indefinitely.

## Technologies Used

* **Rust** programming language
* **Serenity** crate for Discord bot framework
* **ClamAV** for antivirus scanning
* **Regex** for pattern matching masked links

## Status

* Functional as a standalone bot.
* Not yet deployed on a permanent server.
* Anti-phishing feature limited to masked link detection.
* Planned improvements include integration with PhishTank for better phishing URL detection and expanded moderation tools.

## Usage

To run the bot, you need:

* A Discord bot token.
* ClamAV installed and running on your system.
* Rust toolchain to build and run the project.

## Acknowledgements

This project is a personal endeavor to combine Rust programming, Discord bot development, and security practices. Feedback and contributions are welcome as I continue to expand its capabilities.
