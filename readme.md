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


* [ ] Improve Efficiency
	+ [ ] Profile `TreeJump` and `IndexTree`?
* [ ] Different leak function (Hamming distance?)
* [ ] Better structure for different solvers + helper
* [ ] Different cipher-scenarios
	+ [ ] SKINNY
	+ [ ] AES-192
	+ [ ] AES-256
* [ ] Full CLI?
* [x] Simplify the HOF structure for leaks
* [x] Check for further generalization
	+ [x] Structured description of the checks
	+ [x] Implement generalisation
	+ [x] Search generalisation
* [x] Parallelism Code
	+ [x] Implement in Search
* [x] Sanitize inputs (minimal CLI)