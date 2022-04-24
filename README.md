# Exercises to learn substrate

## How to train ?

- fork this repository  
- clone it on your machine  
- switch to the branch of your exercise
- code until all tests passes  

## What is in those exercises ?

We designed the exercises to teach you:
- how to use the numerous features of Substrate
- some good Substrate practices
- some blockchain core principles

Each one exists on two version, on two branches: one for the exercise, with `ex/` prefix, one for the solution, with `solution/` prefix.  
If you are stuck, you can always check the solution. It's not cheating, it's just learning 🧑‍🎓

## Recommended order

- [pallet-testing](https://github.com/rusty-crewmates/substrate-tutorials/tree/ex/pallet-testing): get familiar with substrate testing suite
- [fungible-token](https://github.com/rusty-crewmates/substrate-tutorials/tree/ex/fungible-token): build a basic fungible asset pallet and learn how to write substrate extrinsics and interact with storage
- [nft](https://github.com/rusty-crewmates/substrate-tutorials/tree/ex/nft): build a basic NFT pallet and write more extrinsics (they are substrate 🍞 and 🧈, so better work them twice)
- [marketplace](https://github.com/rusty-crewmates/substrate-tutorials/tree/ex/marketplace): build a marketplace for our NFTs and discover how to make pallets interact with each other

## How to contribute ?

### Create an exercise

- checkout the `main` branch as `solution/<your-exercice-name>`
- code the tests and the solution
- checkout your branch as `ex/<you-exercise-name>`
- remove parts of the code in order to make tests fail
- edit the README with an introduction, add hints and instructions into you code comments
- create two pull requests to this repo

### Feedback

Any feedback is welcome. Feel free to open issues !  
You can also request the themes you would like to be trained on.
