### Secret Network Business Card Demo

This demo showcases the use of Secret Network viewing keys, which are unique, random keys that are created using the Secret Toolkit Random Number Generator (RNG). Each viewing key is created using the current time and the sender of the message as entropy to initialize the random number generator, so that each message has a unique viewing key that is derived from information specific to that message.

To use this dapp:

1. Connect your keplr wallet by clicking the wallet icon (Make sure your dapp has the Secret Network testnet enabled). To add the testnet to your keplr wallet and fund it, [view the Secret Network docs here](https://docs.scrt.network/secret-network-documentation/development/testnet).

2. Create a business card and approve the wallet transaction.

3. Navigate to the "My Cards" tab. In your console, you should see the Viewing Key associated with the business card you created. Enter the wallet address, viewing key, and card number of the card you created to query the card.
