import React, { Component } from 'react';
import logo from './assets/logo.svg';
import nearlogo from './assets/gray_near_logo.svg';
import near from './assets/near.svg';
import './App.css';

class ScholarshipForm extends Component {
  constructor(props) {
    super(props);
    this.state = {
      name: '',
      blockIndex: '',
      result: '',
    };

    this.handleNameChange = this.handleNameChange.bind(this);
    this.handleBlockIndexChange = this.handleBlockIndexChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  async handleSubmit(event) {
    event.preventDefault();
    let result = await this.props.contract.scholarship({name: this.state.name, block_index: parseInt(this.state.blockIndex)}, 100000000000000);
    this.setState({name: '', blockIndex: '', result: result});
  }

  handleNameChange(event) {
    this.setState({name: event.target.value});
  }

  handleBlockIndexChange(event) {
    this.setState({blockIndex: event.target.value});
  }

  render() {
    return (
        <div>
          <p> Check whether you are eligible for scholarship here </p>
          <form onSubmit={this.handleSubmit}>
            <label>
              Name:
            </label>
            <input type="text" label="name" value={this.state.name} onChange={this.handleNameChange} />
            <label>
              BlockIndex:
            </label>
            <input type="text" value={this.state.blockIndex} onChange={this.handleBlockIndexChange} />
            <input type="submit" value="Submit" />
          </form>
          <div>
            <p>
              {this.state.result}
            </p>
          </div>
        </div>
    );
  }
}

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      login: false,
      speech: null
    }
    this.signedInFlow = this.signedInFlow.bind(this);
    this.requestSignIn = this.requestSignIn.bind(this);
    this.requestSignOut = this.requestSignOut.bind(this);
    this.signedOutFlow = this.signedOutFlow.bind(this);
  }

  componentDidMount() {
    let loggedIn = window.walletAccount.isSignedIn();
    if (loggedIn) {
      this.signedInFlow();
    } else {
      this.signedOutFlow();
    }
  }

  async signedInFlow() {
    console.log("come in sign in flow")
    this.setState({
      login: true,
    })
    //const accountId = await this.props.wallet.getAccountId()
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
    //this.props.contract.welcome({ name: accountId }).then(response => this.setState({speech: response.text}))
  }

  async requestSignIn() {
    const appTitle = 'Scholarship Contract';
    await this.props.wallet.requestSignIn(
      window.nearConfig.contractName,
      appTitle
    )
  }

  requestSignOut = () => {
    this.props.wallet.signOut();
    setTimeout(this.signedOutFlow, 500);
    console.log("after sign out", this.props.wallet.isSignedIn())
  }


  signedOutFlow = () => {
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
    this.setState({
      login: false,
      speech: null
    })
  }

  render() {
    let style = {
      fontSize: "1.5rem",
      color: "#0072CE",
      textShadow: "1px 1px #D1CCBD"
    }
    return (
      <div className="App-header">
        <div className="image-wrapper">
          <img className="logo" src={nearlogo} alt="NEAR logo" />
          <p> Private Shard Demo: Scholarship contract</p>
          <p style={style}>{this.state.speech}</p>
        </div>
        <div>
          <ScholarshipForm contract={this.props.contract} />
        </div>
        <div>
          {this.state.login ? <button onClick={this.requestSignOut}>Log out</button>
            : <button onClick={this.requestSignIn}>Log in with NEAR</button>}
        </div>
        <div>
          <div className="logo-wrapper">
            <img src={near} className="App-logo margin-logo" alt="logo" />
            <img src={logo} className="App-logo" alt="logo" />
          </div>
          <p>
            Edit <code>src/App.js</code> and save to reload.
          </p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          <p><span role="img" aria-label="net">🕸</span> <a className="App-link" href="https://nearprotocol.com">NEAR Website</a> <span role="img" aria-label="net">🕸</span>
          </p>
          <p><span role="img" aria-label="book">📚</span><a className="App-link" href="https://docs.nearprotocol.com"> Learn from NEAR Documentation</a> <span role="img" aria-label="book">📚</span>
          </p>
        </div>
      </div>
    )
  }

}

export default App;
