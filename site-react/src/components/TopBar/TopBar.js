import React, { useContext } from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { AuthContext } from '../Auth';
import { makeStyles } from '@material-ui/core/styles';
import { ROUTES } from '../routes';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import Link from '@material-ui/core/Link';
import Box from '@material-ui/core/Box';
import firebase from '../../firebase';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  menuButton: {
    marginRight: theme.spacing(2),
  },
  title: {
    flexGrow: 1,
  },
  topBarItems: {
    display: "flex",
    alignItems: "center",
    "& > *:not(:last-child)": {
      marginRight: 16,
    },
  }
}));

const TopBar = () => {
  const { currentUser } = useContext(AuthContext);
  const classes = useStyles();

  const handleLogout = async () => {
    await firebase.auth().signOut();
  }

  return (
    <div className={classes.root}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" className={classes.title}>
            <Link component={RouterLink} color="inherit" underline="none" to={ROUTES.HOME}>Compilador Funcional</Link>
          </Typography>
          <Box className={classes.topBarItems}>
          {currentUser ?
            (
              <>
                <Typography>Hola, {currentUser.email}</Typography>
                <Button color="inherit" onClick={handleLogout}>Cerrar Sesión</Button>
              </>
            )
          :
            (
              <>
                <Link component={RouterLink} variant="button" color="inherit" underline="none" to={ROUTES.LOGIN}>Iniciar Sesión</Link>
                <Link component={RouterLink} variant="button" color="inherit" underline="none" to={ROUTES.SIGNUP}>Crear Cuenta</Link>
              </>
            )
          }
          </Box>
        </Toolbar>
      </AppBar>
    </div>
  );
}

export default TopBar
