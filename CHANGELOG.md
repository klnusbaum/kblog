# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - yyyy-mm-dd
 
Here we write upgrading notes for brands. It's a team effort to make them as
straightforward as possible.
 
### Added
 - Styling for Blockquotes
 - Ability to list arbitrary links in the footer
 - Ability to include images
 
### Changed
 
### Fixed
 - Fixed light theme CSS (it was using dark colors)
 - removed unused summary field on the RenderedDraft struct
 
## [0.2.0] - 2024-03-19
 
Mostly behind the scenes upgrades to organize the code better.
 
### Added
 
### Changed
 - Upgraded Rust edition to 2021
 - Formatted html files
 - Move to Askama for html templating (#8)
 - Moved to Askama for CSS templating (#7)
 
### Fixed
 - Remove newlines from post summaries (#11)
 - Template out github url (#12)
