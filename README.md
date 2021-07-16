<!-- Title -->
<h1 align="center">
  genomic_structures
</h1>

<!-- description -->
<p align="center">
  <strong>Library for interacting with genomic structures, namely mobile elements and structural variants.</strong>
</p>

<!-- Information badges -->
<p align="center">
  <a href="https://www.repostatus.org/#active">
    <img alt="Repo status" src="https://www.repostatus.org/badges/latest/active.svg?style=flat-square" />
  </a>
  <a href="http://www.gnu.org/licenses/gpl-3.0">
    <img alt="GPL v3" src="https://img.shields.io/badge/License-GPL%20v3-blue.svg">
  </a>
</p>

<!-- Community -->
<p align="center">
  <a href="https://github.com/DanielRivasMD/genomic_structures/discussions">
    <img alt="Ask us anything" src="https://img.shields.io/badge/Ask%20us-anything-1abc9c.svg?style=flat-square">
  </a>
  <a href="https://github.com/SciML/ColPrac">
    <img alt="ColPrac: Contributor's Guide on Collaborative Practices for Community Packages" src="https://img.shields.io/badge/ColPrac-Contributor's%20Guide-blueviolet?style=flat-square">
  </a>
</p>

<!-- Version and documentation badges -->
<p align="center">
  <a href="https://github.com/DanielRivasMD/genomic_structures/releases">
    <img alt="GitHub tag" src="https://img.shields.io/github/v/tag/DanielRivasMD/genomic_structures?label=latest%20version&logo=github&sort=semver&style=flat-square">
  </a>
</p>

<!-- TODO: add logo -->

## Description

**genomic_structures** is a library for interting with genomic structures, namely mobile elements and structural variants.


## Installation

**genomic_structures** is written in Rust, so you'll need to grab a [Rust installation](https://rustup.rs/) in order to compile it.

To build **genomic_structures** :

```
git clone https://github.com/DanielRivasMD/genomic_structures
cd genomic_structures
cargo build --release
```

To integrate **genomic_structures** library into you code, add:

```toml
genomic_structures = "0.1"
```

to your "Cargo.toml" file. To check which element are exposed, view the documentation as indicated below.


## Testing

### Run tests

To run the test suite, use:

```
cargo test
```


## Documentation

To view the documentation, run:

```
cargo doc
```

To open the documentation in your browser, run:

```
cargo doc --open
```


## Citations

If you use **genomic_structures** or derivates in your work, please consider citing the code record.


## Contributing and Support

[![ColPrac: Contributor's Guide on Collaborative Practices for Community Packages](https://img.shields.io/badge/ColPrac-Contributor's%20Guide-blueviolet)](https://github.com/SciML/ColPrac)

In general contributions should follow [ColPrac](https://github.com/SciML/ColPrac). If you are interested in extending/improving **genomic_structures**, head to the [discussions](https://github.com/DanielRivasMD/genomic_structures/discussions) to reach out. For support with using genomic_structures, please open an [issue](https://github.com/DanielRivasMD/genomic_structures/issues/new/) describing the problem and steps to reproduce it.


## License

**genomic_structures** is distributed under the terms of the GNU GENERAL PUBLIC LICENSE.

See [LICENSE](LICENSE) for details.

---

**Author's Note**: This package is still under active development and is subject to change.
