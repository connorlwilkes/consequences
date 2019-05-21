import React, { Component } from 'react';
import Entry from './containers/Entry';
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
        var element = <Entry name="test" />
        // if (!(this.props.loggedIn)) {
        //     element = <Login />
        // }
        return (
            <React.Fragment>
                {element}
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