import React, { Componenet } from 'react';

export default class Index extends React.Component {

    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div>
                <h2>
                    Hello {this.props.name.toString()}
                </h2>
            </div>
        )
    }
}