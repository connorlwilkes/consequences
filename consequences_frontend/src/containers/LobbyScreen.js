import React, { Component } from 'react';
import { TextField } from '@material-ui/core';
import Button from '@material-ui/core/Button';

export default class LobbyScreen extends Component {

    constructor(props) {
        super(props);
        this.state = {lobbyName: ''};
        this.loginRequest = this.createLobby.bind(this);
        this.handleChange = this.handleChange.bind(this);
    }

    render() {
        return (
            <React.Fragment>
            <form onSubmit={this.loginRequest}>
                <TextField label="Lobby name" onChange={this.handleChange} />
                <Button variant="contained" type="submit" color="primary"> 
                    
                </Button>
            </form>
        </React.Fragment>
        ) 
    }

    handleChange(event) {
        this.setState({lobbyName: event.target.value});
    }

    createLobby(event) {
        console.log(this.state.username);
        fetch("http://localhost:8080/create-lobby", {
            credentials: "include",
            method: 'POST',
            body: JSON.stringify({lobby_name: this.state.lobbyName}),
            headers: {
                'Content-type': 'application/json'
            }
        }).then(res => res.json())
        .then(response => console.log('Success:', JSON.stringify(response)))
        .catch(error => console.log('Error:', error))
        event.preventDefault();
    }

}