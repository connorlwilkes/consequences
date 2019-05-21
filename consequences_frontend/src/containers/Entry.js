import React, { Componenet } from 'react';

export default class Index extends React.Component {

    constructor(props) {
        super(props)
        console.log(props)
    }

    render() {
        return (
            <div>
                <h2>
                    Hello
                    {this.props.loggedIn}
                </h2>
            </div>
        )
    }
}