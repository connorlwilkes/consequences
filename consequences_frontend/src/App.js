import React, { Component } from 'react';
import ReactDom from 'react-dom';
import Entry from './containers/Entry';
import SubmitButton from './components/submitButton';

export default class App extends Component {

    constructor(props) {
        super(props);
        this.state= {
            loggedIn: false,
        };
    }

    render () {
        return (
            <React.Fragment>
                <Entry
                    loggedIn={this.state.loggedIn}
                />
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
            console.log(body)
            let str = String(body)
            if (str.length > 0) {
                this.setState({ loggedIn: true })
            }
            console.log(this.state.loggedIn)
        })
        .catch(err => console.log(err))
    }
}