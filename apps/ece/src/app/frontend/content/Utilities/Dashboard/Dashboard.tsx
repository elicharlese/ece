import React from 'react';
import { Container, Grid, Paper, Box, Typography } from '@mui/material';
import { makeStyles } from '@mui/styles';

const useStyles = makeStyles({
  root: {
    flexGrow: 1,
    padding: '2rem',
  },
  paper: {
    padding: '1rem',
    textAlign: 'center',
    color: '#333',
  },
});

const Dashboard: React.FC = () => {
  const classes = useStyles();

  return (
    <Container className={classes.root}>
      <Typography variant="h4" gutterBottom>
        Dashboard
      </Typography>
      <Grid container spacing={3}>
        <Grid item xs={12} sm={6} md={4}>
          <Paper className={classes.paper}>Item 1</Paper>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Paper className={classes.paper}>Item 2</Paper>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Paper className={classes.paper}>Item 3</Paper>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Paper className={classes.paper}>Item 4</Paper>
        </Grid>
      </Grid>
    </Container>
  );
};

export default Dashboard;