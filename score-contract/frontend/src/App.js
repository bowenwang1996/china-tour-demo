import React, { Component } from 'react';
import logo from './assets/logo.svg';
import nearlogo from './assets/gray_near_logo.svg';
import near from './assets/near.svg';
import './App.css';

class ScoreForm extends Component {
  constructor(props) {
    super(props);
    this.state = {
      name: '',
      score: '',
    };

    this.handleNameChange = this.handleNameChange.bind(this);
    this.handleScoreChange = this.handleScoreChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  async handleSubmit(event) {
    event.preventDefault();
    console.log("name: " + this.state.name + ", score: " + this.state.score);
    await this.props.contract.record_score({name: this.state.name, score: parseInt(this.state.score)}, 100000000000000);
    this.setState({name: '', score: ''});
  }

  handleNameChange(event) {
    this.setState({name: event.target.value});
  }

  handleScoreChange(event) {
    this.setState({score: event.target.value});
  }

  render() {
    return (
        <form onSubmit={this.handleSubmit}>
          <label>
            Name:
            <input type="text" value={this.state.name} onChange={this.handleNameChange} />
          </label>
          <label>
            Score:
            <input type="text" value={this.state.score} onChange={this.handleScoreChange} />
          </label>
          <input type="submit" value="Submit" />
        </form>
    );
  }
}

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      login: false,
      speech: null
    };
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
    });
    if (window.location.search.includes("account_id")) {
      window.location.replace(window.location.origin + window.location.pathname)
    }
  }

  async requestSignIn() {
    const appTitle = 'NEAR React template';
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
          <p> Private Shard demo: A shard that manages scores </p>
          <p style={style}>{this.state.speech}</p>
        </div>
        <div className="score-form">
          <ScoreForm contract={this.props.contract} />
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
