import React, { Component, Fragment } from 'react';
import SubmitButton from '../components/submitButton';
import { TextField } from '@material-ui/core';
import Button from '@material-ui/core/Button';

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
                <React.Fragment>
                    <form onSubmit={this.loginRequest}>
                        <TextField label="Username" onChange={this.handleChange} />
                        <Button variant="contained" type="submit" color="primary"> 
                            Submit
                        </Button>
                    </form>
                </React.Fragment>
          );
    }

    loginRequest(event) {
        console.log(this.state.username);
        fetch("http://localhost:8080/login", {
            credentials: "include",
            method: 'POST',
            body: JSON.stringify({name: this.state.username}),
            headers: {
                'Content-type': 'application/json'
            }
        }).then(res => res.json())
        .then(response => console.log('Success:', JSON.stringify(response)))
        .catch(error => console.log('Error:', error))
        event.preventDefault();
    }

}