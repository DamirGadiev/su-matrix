### Resistance calculation for electron interacting with one-dimensional wire.

This program allows to calculate resistance of the one-dimensional wire to the electron.
Transfer-matrix method is used to achieve the goal.
Source investigation: http://sn-physmat.crimea.edu/arhiv/2012/uch_25_1/003_axr.pdf

Resulting plots:
- Dependency of the resistance from the distance between scattering potentials:
http://i.prntscr.com/Ezlcm3MyQBW0XxDqTsHRag.png
- Dependency of the resistance from the energy of an electron:
http://i.prntscr.com/eQB6kD28T2y5JD40eYolLA.png
- Dependency of the resistance from the potential of the scattering point:
http://i.prntscr.com/4CoNHW88Q3mXVL-5LGfqVQ.png

Results:
- Resistance grows when potential energy of the scattering point grows.
- Resistance becomes less when energy of the moving electron grows.
- Localized states appears, as there is strict dependency between resistance and length between atoms.

Most urgent TODO:
- Add ability to run with arguments and write results to the file.
- Rename repo and the module.
- Split functionality to modules
- Create struct abstractions to simplify cases modification.
- Add lightweight disorder.
- Add hard disorder.
- Move to 2D scattering.

Less urgent TODO:
- Add plotting out of the box.
