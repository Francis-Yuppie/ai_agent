# CipherMind - AI-Powered GitHub Code Analyzer for ICP

CipherMind is an Internet Computer (ICP) canister that leverages AI, specifically ICP LLM , to scan and monitor deployed canisters on ICP alos analyze GitHub repositories for security vulnerabilities and code quality issues. It provides developers with automated code scanning directly on the ICP, enhancing the security and reliability of their projects.

## Features

- Security & Vulnerability Detection --dentifies potential security flaws in the codebase.
- Code Quality Evaluation
- Automated Code Review Suggestions
- Basic Code Quality Analysis (Placeholder):
  Provides a foundation for future code quality metric implementations.
- ICP Canister Integration:
  Runs as an ICP canister, enabling decentralized and secure code analysis.
- Dependency Analysis Examines dependencies for outdated or vulnerable packages.
  Smart Insights

- Uses AI models to predict potential future issues based on coding patterns.

- Customizable Rules & Configurations

  Allows users to customize AI analysis parameters.

## Getting Started

### Prerequisites

- DFX (Internet Computer SDK) installed.
- Node.js and npm installed (for frontend, if applicable).
- A GitHub repository URL to analyze.
- An OpenAI API key.

### Deployment

1.  **Clone the Repository:**

    ```bash
    git clone [your-CipherMind-repository-url]
    cd CipherMind/
    ```

2.  **Start the Local Replica (for testing):**

    ```bash
    dfx start --background
    ```

3.  **Deploy the Canister:**

    ```bash
    dfx deploy
    ```

    - This command deploys your canister to the local replica and generates the Candid interface.
    - Make sure you have added your OpenAI API key in the code.
    - Also make sure that you have added your chat gpt endpoint in the code.

4.  **Interact with the Canister:**

    - You can interact with the canister using `dfx canister call` or through a frontend interface.
    - To analyze a GitHub repository, call the `analyze_repo` function:

    ```bash
    dfx canister call CipherMind_backend analyze_repo '(record { repo_url = "[https://github.com/your-username/your-repo](https://www.google.com/search?q=https://github.com/your-username/your-repo)" })'
    ```

5.  **View the Analysis Report:**

    - The analysis report will be returned by the `analyze_repo` function.
    - You can integrate this report into a frontend application for a better user experience.

### Frontend Development (Optional)

1.  **Start the Frontend Server:**

    ```bash
    npm start
    ```

    - This will start a development server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Candid Interface Generation

If you make changes to the canister's Candid interface, regenerate it with:

```bash
npm run generate
```
