   // src/utils/stripe.js
   import Stripe from 'stripe';

   const stripe = new Stripe('your_secret_key_here', {
       apiVersion: '2022-11-15',
   });

   export default stripe;