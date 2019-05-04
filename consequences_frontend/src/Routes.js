import React from 'react';
import { Route, Switch } from 'react-router-dom';
import LandingPage from './containers/LandingPage';
import NotFound from './containers/404';
import Login from "./containers/Login";

export default () =>
    <Switch>
        <Route path="/" exact component={LandingPage} />
        <Route path="/login" exact component={Login} />
        <Route component={NotFound} />
    </Switch>;
