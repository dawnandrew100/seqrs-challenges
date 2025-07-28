## July challenge

**Strobemers** are fuzzy matching seeds that offer better tolerance for mutations
like insertions and deletions compared to traditional k-mers. They're a great
tool for efficient sequence comparison and alignment.

Implement your own **strobemer generator function** in Rust.

- MinStrobe - Choose subsequent strobes by selecting the minimum hash within a
sliding window.
- RandStrobe - Choose subsequent strobes by a pseudo-random but reproducible rule
derived from hash values.

**Your function should**:

- Input: DNA string
- Parameters: `k`, `w_min`, and `w_max` (k = strobe length, w = window length)
- Output: a list of strobemers (strings or hashes)

### Bonus Challenge

Invent and implement a **new strobe algorithm**!

Here are a few ideas to inspire you:

- **HybridStrobe** – switch between minstrobe and randstrobe dynamically
- **LoopStrobe** – pick strobes from both forward and reverse directions
- **EntropyStrobe** – prefer high-complexity regions for matching
- **FixedGapStrobe** – always pick the second strobe at a fixed offset