# zkChess 🎮♟️

A zero-knowledge proof implementation of chess move validation, built for the Aligned Hackathon 2024.

## Project Description

zkChess is a revolutionary approach to chess move validation using zero-knowledge proofs. In traditional online chess, players must trust the server to validate moves correctly and store the solution. With zkChess, move that solve a puzzle is performed inside a zkVM (Zero-Knowledge Virtual Machine), providing cryptographic guarantees.

After you solve the challenge you can mint a NFT.

### Why zkChess?
Chess has been a testing ground for computational advances for decades. By implementing chess move validation in a zkVM, we're demonstrating how complex game logic can be verified without revealing the underlying state.

## Team

- Fiorella
  - ZK Entusiast


## Technical Challenges & Design Considerations

### Challenges Faced

1. **State Representation**
   - Efficiently encoding the chess board state
   - Minimizing circuit complexity for move validation
   - Balancing between storage efficiency and proof generation speed

2. **Engine**
   - Running an engine in SP1 is complex, sadly my first approach didnt work :(
  
3. **Zero-Knowledge Integration**
   - Adapting traditional chess logic to work within zkVM constraints
   - Managing state transitions in a provable manner

### Design Decisions

1. **Proof System Architecture**
   - Built on SP1 zkVM for robust zero-knowledge capabilities
   - Modular design allowing for future optimizations
   - Focused on minimizing the proving time while maintaining security

## Deployment & Execution

### Running the Project
```shell
make all

cd script
./target/release/prove --board '{"w":["Ke1","Qf3","Bc4","Pd2","Pe4"],"b":["Kf8","Pf7","Pg7","Ph7","Pe7"]}' --mode interactive --moves '[]' --keystore-path ~/.foundry/keystores/fio
```


## Project Roadmap

### Phase 1: Core Implementation (Complete)
- ✅ Zero-knowledge proof integration
- ✅ Test suite development

### Phase 2: Enhancement (In Progress)
- 🔄 Basic chess move validation
- 🔄 Implementation of simple engine

### Phase 3: Future Development
- 📅 Public curated puzzles database
- 📅 Open games agaisnt ccmputer

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.txt) file for details.

---

*Built with ♟️ by the zkChess team for the Aligned Hackathon 2024*
