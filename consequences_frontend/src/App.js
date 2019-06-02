import React, { Component } from 'react';
import Index from './containers/Entry';
import Login from './containers/Login';

export default class App extends Component {

    constructor(props) {
        super(props);
        this.state= {
            loggedIn: false,
            name: "test",
        };
    }

    render () {
        return (
            <React.Fragment>
                <Index />
            </React.Fragment>
          );
    }

    componentDidMount() {
        this.setState()
        fetch("http://localhost:8080/", {
            credentials: "include",
            method: 'GET',
            headers: {
                'Content-type': 'application/json'
            }
        }).then(res => res.text())
        .then(body => {
            let str = String(body)
            if (str.length > 0) {
                this.setState({ 
                    loggedIn: true,
                    name: str,
                 })
            }
        })
        .catch(err => console.log(err))
    }
}