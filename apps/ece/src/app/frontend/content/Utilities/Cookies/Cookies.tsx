import React, { useState, useEffect } from 'react';
import { useCookies } from 'react-cookie';
import { Box, Button, Typography } from '@mui/material';
import "./Cookies.scss"; // Import the SCSS file

const CookiesBanner: React.FC = () => {
  const [cookies, setCookie] = useCookies(['user-consent']);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    if (!cookies['user-consent']) {
      setIsVisible(true);
    }
  }, [cookies]);

  const acceptCookies = () => {
    setCookie('user-consent', 'accepted', { path: '/', maxAge: 31536000 });
    setIsVisible(false);
  };

  const declineCookies = () => {
    setIsVisible(false);
  };

  if (!isVisible) {
    return null;
  }

  return (
    <Box className="banner">
      <Typography variant="body1">
        We use cookies to improve your experience on our site. By accepting, you
        agree to our use of cookies.
      </Typography>
      <Box>
        <Button color="primary" variant="contained" onClick={acceptCookies}>
          Accept
        </Button>
        <Button className="button" color="secondary" variant="contained" onClick={declineCookies}>
          Decline
        </Button>
      </Box>
    </Box>
  );
};

export default CookiesBanner;