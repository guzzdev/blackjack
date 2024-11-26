const suits = ['Hearts', 'Diamonds', 'Clubs', 'Spades'];
const values = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'];
const suitSymbols = {
    'Hearts': '♥',
    'Diamonds': '♦',
    'Clubs': '♣',
    'Spades': '♠'
};

let deck = [];
let playerHand = [];
let dealerHand = [];
let dealerHidden = true;
let playerMoney = 100;
let currentBet = 0;
let wins = 0;
let losses = 0;

function createDeck() {
    deck = [];
    for (let suit of suits) {
        for (let value of values) {
            deck.push({ suit, value });
        }
    }
}

function shuffleDeck() {
    for (let i = deck.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [deck[i], deck[j]] = [deck[j], deck[i]];
    }
}

function getCardValue(card) {
    if (card.value === 'A') return 11;
    if (['K', 'Q', 'J'].includes(card.value)) return 10;
    return parseInt(card.value);
}

function calculateHandValue(hand) {
    let value = 0;
    let aceCount = 0;
    for (let card of hand) {
        value += getCardValue(card);
        if (card.value === 'A') aceCount++;
    }
    while (value > 21 && aceCount > 0) {
        value -= 10;
        aceCount--;
    }
    return value;
}

function dealCard(hand) {
    hand.push(deck.pop());
}

function startGame() {
    createDeck();
    shuffleDeck();
    playerHand = [];
    dealerHand = [];
    dealerHidden = true;
    dealCard(playerHand);
    dealCard(dealerHand);
    dealCard(playerHand);
    dealCard(dealerHand);
    updateUI();
}

function updateUI() {
    const playerCardsDiv = document.getElementById('player-cards');
    const dealerCardsDiv = document.getElementById('dealer-cards');
    const playerValueDiv = document.getElementById('player-value');
    const dealerValueDiv = document.getElementById('dealer-value');
    const playerMoneyDiv = document.getElementById('player-money');
    const playerMoneyDiffDiv = document.getElementById('player-money-diff');
    const winsDiv = document.getElementById('wins');
    const lossesDiv = document.getElementById('losses');
    const messageDiv = document.getElementById('message');
    
    playerCardsDiv.innerHTML = '';
    dealerCardsDiv.innerHTML = '';
    playerHand.forEach(card => {
        playerCardsDiv.innerHTML += `<div class="card ${card.suit.toLowerCase()} card-deal"><div class="value">${card.value}</div><div class="suit">${suitSymbols[card.suit]}</div></div>`;
    });
    dealerHand.forEach((card, index) => {
        if (index === 1 && dealerHidden) {
            dealerCardsDiv.innerHTML += `<div class="card hidden card-deal">Hidden</div>`;
        } else {
            dealerCardsDiv.innerHTML += `<div class="card ${card.suit.toLowerCase()} card-deal"><div class="value">${card.value}</div><div class="suit">${suitSymbols[card.suit]}</div></div>`;
        }
    });
    playerValueDiv.innerText = `Player: ${calculateHandValue(playerHand)}`;
    dealerValueDiv.innerText = dealerHidden ? `Dealer: ${getCardValue(dealerHand[0])}` : `Dealer: ${calculateHandValue(dealerHand)}`;
    playerMoneyDiv.innerText = `Money: ${playerMoney}€`;
    playerMoneyDiffDiv.innerText = (playerMoney >= 100 ? '+' : '') + (playerMoney - 100) + '€';
    winsDiv.innerText = `Wins: ${wins}`;
    lossesDiv.innerText = `Losses: ${losses}`;
}

function checkGameOver() {
    if (playerMoney <= 0) {
        document.getElementById('message').innerText = 'Game Over! You have no more money.';
        document.getElementById('hit-button').disabled = true;
        document.getElementById('stand-button').disabled = true;
        document.getElementById('place-bet-button').disabled = true;
        alert('Game Over! You have no more money.');
    }
}

document.getElementById('hit-button').addEventListener('click', () => {
    dealCard(playerHand);
    if (calculateHandValue(playerHand) > 21) {
        document.getElementById('message').innerText = 'Player busts! Dealer wins!';
        document.getElementById('hit-button').disabled = true;
        document.getElementById('stand-button').disabled = true;
        playerMoney -= currentBet;
        losses++;
        checkGameOver();
        setTimeout(() => {
            document.getElementById('place-bet-button').disabled = false;
        }, 2000);
    }
    updateUI();
});

document.getElementById('stand-button').addEventListener('click', () => {
    dealerHidden = false;
    while (calculateHandValue(dealerHand) < 17) {
        dealCard(dealerHand);
    }
    const playerValue = calculateHandValue(playerHand);
    const dealerValue = calculateHandValue(dealerHand);
    let message = '';
    if (dealerValue > 21 || playerValue > dealerValue) {
        message = 'Player wins!';
        playerMoney += currentBet;
        wins++;
    } else if (playerValue < dealerValue) {
        message = 'Dealer wins!';
        playerMoney -= currentBet;
        losses++;
    } else {
        message = 'It\'s a tie!';
    }
    document.getElementById('message').innerText = message;
    document.getElementById('hit-button').disabled = true;
    document.getElementById('stand-button').disabled = true;
    checkGameOver();
    setTimeout(() => {
        document.getElementById('place-bet-button').disabled = false;
    }, 2000);
    updateUI();
});

document.getElementById('place-bet-button').addEventListener('click', () => {
    currentBet = parseInt(document.getElementById('bet-amount').value);
    if (currentBet > playerMoney) {
        document.getElementById('message').innerText = 'Insufficient funds!';
    } else {
        document.getElementById('message').innerText = ''; // Clear the message
        document.getElementById('hit-button').disabled = false;
        document.getElementById('stand-button').disabled = false;
        document.getElementById('place-bet-button').disabled = true;
        startGame();
    }
});

updateUI();