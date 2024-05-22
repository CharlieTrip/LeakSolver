# LeakSolver

**WIP**: check my [webpage](https://charlietrip.neocities.org/projects/sca-2aes) for more info.

## TODO (readability)

* [ ] Write documentation
* [ ] Add comments
* [ ] Clean up the code-structure
* [ ] Complete AES tests:
	+ [ ] key scheduler
	+ [ ] parallel sbox
	+ [ ] mixing layer
* [ ] Complete AESGen tests
* [x] Write explanations (blog)

## TODO (algorithmic)

* [x] Simplify the HOF structure for leaks
* [ ] Different leak function (Hamming distance?)
* [ ] Different cipher-scenarios
	+ [ ] SKINNY
	+ [ ] AES-192
	+ [ ] AES-256
* [ ] Full CLI?
* [x] Check for further generalization
	+ [x] Structured description of the checks
	+ [ ] Implement generalisation
	+ [ ] Compute leaks by unrolling full round
* [ ] Parallelism Code
	+ [ ] Toy Example for search
	+ [ ] Implement in Search
* [x] Sanitize inputs (minimal CLI)