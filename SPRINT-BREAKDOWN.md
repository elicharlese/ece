# Sprint Breakdown for Cleanfolio Project

## Day 1: Environment Setup and Initial Planning

- **Total Story Points:** 20
  - [x] Set up the Next.js project with TypeScript.
  - [ ] Integrate TailwindCSS and Sass for styling.
  - [ ] Prepare Docker environment for containerization.
  - [ ] Initialize pnpm workspace.
  - [ ] Scaffold basic file structure (components, pages, utilities).
  - [ ] Create basic theming components for dark/light mode toggling & state management.

  - [ ] Develop a comprehensive feature roadmap
  - [ ] Outline smart contract requirements for NEAR Rust SDK.
  - [ ] Define data models and interfaces for wallet integration and crowd funding center.

## Day 2-4: Smart Contracts and Basic UI Layout

- **Total Story Points:** 55
  - [ ] Begin coding smart contracts using NEAR Rust SDK.
  - [ ] Develop test cases for smart contracts.
  - [ ] Home Page layout.
  - [ ] Marketplace skeleton interface.
  - [ ] Stub out UI components for wallet functionality.
  - [ ] Integrate Thirdweb for Web3 functionalities.
  - [ ] Implement Wallet connection and display user's ERC20 token balance.
  - [ ] Implement purchase/swap functions in smart contracts.
  - [ ] Continue writing test cases and perform initial testing.
  - [ ] Style Home Page and Marketplace with TailwindCSS and Sass.
  - [ ] Add responsive design elements for better mobile support.

## Day 5-7: Crowdfunding Center and Application Shells

- **Total Story Points:** 25
  - [ ] Scaffold backend logic for crowdfunding mechanism.
  - [ ] Design database schema for tracking contributions.
  - [ ] Create shell applications within the platform (TxnTracker, SocketChat, etc.).
  - [ ] Define routing and base components for each application.

## Day 8-13: Applications Development Start

- **Total Story Points:** 50
  - **TxnTracker**: (4 points)
    - [ ] Implementation of transaction tracking logic.
    - [ ] UI development for transaction history.

  - **SocketChat**: (4 points)
    - [ ] Set up WebSocket server.
    - [ ] Implement basic chat interface.

  - **NanoNode**: (4 points)
    - [ ] Initial setup of microservice for node communication.
    - [ ] Interface to visualize node status.

  - **Bitcellular**: (4 points)
    - [ ] Define cellular automata rules and interactions.
    - [ ] Build visual representation for automata state.

  - **ThePublic**: (4 points)
    - [ ] Initiate a feed or forum-like component.
    - [ ] Determine moderation and posting features.

  - **Applications Development Finish**: (15 points)
    - [ ] Complete any remaining logic and interfaces for TxnTracker, SocketChat, NanoNode, Bitcellular, and ThePublic.
    - [ ] Ensure all apps are integrated and functioning within the overall platform.

  - **Integration and End-to-End Testing**: (10 points)
    - [ ] Test integration of all components, applications, and smart contract interactions.
    - [ ] Debug issues and refine user experience.

  - **Documentation and Cleanup**: (5 points)
    - [ ] Write developer and user documentation for the platform.
    - [ ] Code cleanup and optimization.

## Day 14: Final Touches and Deployment

- **Total Story Points:** 20
  - [ ] Conduct thorough UAT with potential users.
  - [ ] Gather feedback and make necessary adjustments.
  - [ ] Perform security audits on smart contracts.
  - [ ] Address identified vulnerabilities.
  - [ ] Containerize final application with Docker for deployment.
  - [ ] Set up CI/CD pipeline if required.
  - [ ] Deploy application to production.
  - [ ] Monitor initial launch for urgent issues and fix as needed.
