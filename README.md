<!-- PROJECT LOGO -->
<div align="center">
  <h3 align="center">Ord20 Indexer (ord-20) library</h3>

  <p align="center">
    This indexer fully implements the ordinals meta protocol of brc-20/ltc-20/drc-20 to hereby be known as ord-20.
    <br />
    <br />
    <a href="https://github.com/ynohtna92/ord20-indexer/issues">Report Bug</a>
    ·
    <a href="https://github.com/ynohtna92/ord20-indexer/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

Developers of the meta protocol ord-20 can integrate this indexer in the code according to their needs.

Once the indexer is started and connected to a local ordinals server, it will process each block one by one and populate the postgreSQL database with a detailed list of all token information, holder balances, and inscription transfer history.

<!-- GETTING STARTED -->
## Getting Started
Ord20 Indexer and its prerequisites can be installed from the command line.

### Prerequisites
1. Install Rust
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    This project is developed with rust and can be built from source.

2. Install PostgreSQL
* PostgreSQL (Linux)
  ```sh
  sudo apt-get install postgresql postgresql-client
  ```
  Start PostgreSQL CLI as the `postgres` user
  ```
  sudo -u postgres psql
  ```
  Modify the PostgreSQL password
  ```
  sudo -u postgres psql
  \\password
  ```
* PostgreSQL (Mac)
  ```sh
  brew install postgresql
  ```
  Start the server:
  ```
  brew services start postgresql
  ```

3. Install Diesel
    ```sh
   cargo install diesel_cli --no-default-features --features postgres
   ```

### Installation

1. Clone the repo and build the binary
   ```sh
   git clone https://github.com/ynohtna92/ord20-indexer.git
   cd ord20-indexer
   cargo build --release
   ```

2. Download Litecoin core 0.21.2.2 from official website https://litecoin.org/
    ```sh
    tar -zxvf litecoin-0.21.2.2-x86_64-linux-gnu.tar.gz
    sudo install -m 0755 -o root -g root -t /usr/local/bin ~/litecoin-0.21.2.2/bin/*
    ```

3. Edit the `litecoin.conf` file of the full node
    ```
    cd ~/.litecoin
    vi litecoin.conf
    ```
    Add the following configurations and save
    ```
    txindex=1
    rpcserialversion=1
    ```

5. Run the Litecoin full node until it is fully synchronised (depend on your network, it will generally take up to several hours to days to sync)

6. Clone the ordinals repo and build the ord server.
    ```sh
    git clone https://github.com/ynohtna92/ord-litecoin.git
    cd ord-litecoin
    git checkout api
    cargo build --release
    ```

7. Run the ordinals server, and allow it to fully synchronise.
    ```
    ./ord server
    ```

8. Configure the ord20-indexer in your `.env` file.
    ```
    # The ordinals metaprotocol to index (brc-20, ltc-20, drc-20)
    META_PROTOCOL=ltc-20
    
    # Starting index block height
    START_BLOCK=2465225
    
    # How many blocks to stay behind to reduce the impact of block reorganisations (0 = disabled, 6 = recommended)
    BLOCKS_BEHIND=6
    
    # The ordinals server address (https://ordinals.com/, https://ordinalslite.com/, http://localhost/)
    ORDINALS_BASE_URL=http://localhost/
    
    # The postgres database credentials and url
    DATABASE_URL=postgres://postgres:password@localhost/ord20-indexer
    ```

9. Run the database installation scripts
    ```
    diesel migration run
    ```

10. Run the ord-20 indexer
    ```
    ./ord20-indexer
    ```

<!-- ROADMAP -->
## Roadmap

- [ ] Extend the application to cover multiple ordinal meta protocols.
- [ ] Index directly from full nodes using the binary data found in blocks and transactions

See the [open issues](https://github.com/ynohtna92/ord20-indexer/issues) for a full list of proposed features (and known issues).

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the GNU v3 License. See `LICENSE.txt` for more information.

<!-- CONTACT -->
## Contact

Twitter/X - [@anthonyonchain](https://twitter.com/anthonyonchain)

Project Link: [https://github.com/ynohtna92/ord20-indexer.git](https://github.com/ynohtna92/ord20-indexer.git)

<p align="right">(<a href="#readme-top">back to top</a>)</p>