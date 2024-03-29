# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2023-03-11
### Added
 - Add regression test
### Changed
 - Change edition to 2021
 - Update image to 0.23.14

## [0.1.1] - 2020-08-20
### Added
 - Sample images
 - Regression test
 - Addressed clippy lints
 - This CHANGELOG file

## [0.1.0] - 2020-08-16
### Added
 - README
 - `pub fn map_normals(img: &DynamicImage) -> RgbImage`
 - `pub fn map_normals_with_strength(img: &DynamicImage, strength: f32) -> RgbImage`
 - `pub const DEFAULT_STRENGTH: f32 = 6.0`
 - CLI utility
 - MIT License
