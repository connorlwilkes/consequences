import React, { Component } from 'react';
import Button from '@material-ui/core/Button';

export default class SubmitButton extends Component {
    
    constructor(props) {
        super(props);
        this.state = {
            pressed: false
        };
    }

    render () {
        return (
            <Button variant="contained" component="span" style={{ margin: 20 }}>
                Submit
            </Button>
        );
    }
} 