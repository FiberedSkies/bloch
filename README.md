# Bloch

[GitHub Actions](https://github.com/0xEstelle/bloch/workflows/test/badge.svg)

Some experiments on applications of fibered categories and torsors in state tomography for the spin-1/2 quantum system. The goal of this repo is largely just to play around and see what kind of insights can be gained from this perspective, with an aim to eventual expand and apply this to state tomography for higher-dimensional systems (such as multi-qubit arrays). Some goals for this project include:

- [ ] Implement a base site for the spin-1/2 system's state space (ideally both the Bloch sphere and fibrations over it, as both perspectives are useful. These both are topological spaces so the site is really just the category of open sets with the appropriate morphisms)
- [ ] Implement a fibered category over the site, with fibers as the subcategory of all measurement data over the site indexed by our observable operators. This will be the basis for defining our stack (which wil involve further defining the descent conditions for the measurement data)
- [ ] Investigate the application of this in state tomography, and see if we can gain any insights from this perspective that are not immediately obvious from the standard perspective of density matrices and POVMs (this is a much more broad goal, but I think it's a good one to keep in mind)

### Why Rust?

Why not.