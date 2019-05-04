import React, { Component } from 'react';
import Routes from "./Routes";

class App extends Component {

    constructor(props) {
        super(props);
        this.state= {
            isAuthenticated: false
        };
    }

    userHasAuthenticated() {
        this.setState({isAuthenticated: true});
    }

    render () {
        const childProps = {
            isAuthenticated: this.state.isAuthenticated,
            userHasAuthenticated: this.userHasAuthenticated
        };

        return (
            <div className="App container">
                <Routes childProps={childProps}/>
            </div>
          );
    }
}


export default App;
