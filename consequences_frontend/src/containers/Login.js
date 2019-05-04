import React, { Component } from 'react';

export default class LoginForm extends Component {

    constructor(props) {
        super(props);
        this.state = {username: ''};
        this.loginRequest = this.loginRequest.bind(this);
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        this.setState({username: event.target.value});
    }

    render () {
        return (
            <form onSubmit={this.loginRequest}>
                <label>
                    Name:
                    <input type="text" value={this.state.username} onChange={this.handleChange} />
                </label>
                <input type="submit" value="Submit" />
          </form>
          );
    }

    loginRequest(event) {
        console.log(this.state.username);
        fetch("http://localhost:8080/login", {
            method: 'POST',
            body: JSON.stringify({name: this.state.username}),
            headers: {
                'Content-type': 'application/json'
            }
        }).then(res => res.json())
        .then(response => console.log('Success:', JSON.stringify(response)))
        .catch(error => console.log('Error:', error));
        event.preventDefault();
    }

}