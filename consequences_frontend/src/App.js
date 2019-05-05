import React, { Component } from 'react';
import ReactDom from 'react-dom';
import Login from './containers/Login';
import SubmitButton from './components/submitButton';

export default class App extends Component {

    constructor(props) {
        super(props);
        this.state= {
            isAuthenticated: false
        };
    }

    render () {
        return (
            <React.Fragment>
                <Login />
            </React.Fragment>
          );
    }
}
