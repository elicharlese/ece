const nearService = require('../services/nearService');

exports.registerUser = async (req, res) => {
  try {
    const { accountId, username } = req.body;
    // Register user in the database or NEAR contract
    const user = await nearService.registerUser(accountId, username);
    res.status(200).json(user);
  } catch (error) {
    console.error('Registration failed', error);
    res.status(500).send('Registration failed');
  }
};

exports.getUser = async (req, res) => {
  try {
    const { accountId } = req.params;
    // Fetch user data from the database or NEAR contract
    const user = await nearService.getUser(accountId);
    res.status(200).json(user);
  } catch (error) {
    console.error('Fetching user failed', error);
    res.status(500).send('Fetching user failed');
  }
};