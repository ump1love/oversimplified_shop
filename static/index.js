const purchasedProducts = document.getElementById('purchased-products');
const productList = document.getElementById('product-list');
const tableItems = document.getElementById('cart-items');
const outputTotalPrice = document.getElementById('total-price');

let purchaseAmount = 0;
let purchasedItems = new Map();

fetch('static/products.json') 
    .then(response => response.json())
    .then(data => {
        console.log(data);
        productsHandler(data);
    })
    .catch(error => {
      console.error('Error fetching the JSON file:', error);
    });

function productsHandler(products) {
    listProducts(products);

    const cartButton = document.getElementById('cart-button');
    const closeButton = document.getElementById('close-popup');

    cartButton.addEventListener('click', () => {
        tableItems.innerHTML = '';
        outputTotalPrice.innerHTML = '';
        document.getElementById('cart-popup').style.display= "block";
        cartPopup(products);
    });

    closeButton.addEventListener('click', () => {
        document.getElementById('cart-popup').style.display = "none";
        tableItems.innerHTML = '';
        outputTotalPrice.innerHTML = '';
    });
}

function calculateTotalPrice(products) {
    let totalPrice = 0;
    purchasedItems.forEach((quantity, id) => {
        const product = products.find(product => product.id === id);
        if (product) {
            totalPrice += product.price * quantity;
        }
    });
    return totalPrice;
}

function calculateSingleProductPrice(products, id) {
    let totalPrice = 0;
    const productInfo = products.find(p => p.id === id);
    if (productInfo) {
        totalPrice += productInfo.price * purchasedItems.get(id);
    }
    return totalPrice;
}

function cartPopup(products) {
    let totalPrice = calculateTotalPrice(products);
    const totalPriceElement = document.createElement('p');
    totalPriceElement.textContent = `Total Price: $${totalPrice}`;

    purchasedItems.forEach((quantity, id) => {
        let singleProductPrice = calculateSingleProductPrice(products, id);
        const product = products.find(p => p.id === id);

        if (!product) return;

        const tableRow = document.createElement('tr');
        const nameCell = document.createElement('td');
        nameCell.textContent = product.name;

        const quantityCell = document.createElement('td');
        quantityCell.textContent = quantity;

        const priceCell = document.createElement('td');
        priceCell.textContent = `$${singleProductPrice}`;

        tableRow.appendChild(nameCell);
        tableRow.appendChild(quantityCell);
        tableRow.appendChild(priceCell);

        tableItems.appendChild(tableRow);
    });

    outputTotalPrice.appendChild(totalPriceElement);
}

function listProducts(products) {
    products.forEach(product => {
        const cartButton = document.getElementById('cart-button');

        const productItem = document.createElement('div');
        productItem.className = 'product-item';

        const nameAndPrice = document.createElement('p');
        nameAndPrice.textContent = `${product.name} - $${product.price}`;

        const productQuantity = document.createElement('span');
        productQuantity.textContent = `Quantity: ${product.quantity}`;

        const productPurchaseButton = document.createElement('button');
        productPurchaseButton.textContent = 'Purchase';
        productPurchaseButton.className = 'purchase-button';

        productPurchaseButton.addEventListener('click', () => {
            if (product.quantity > 0) {
                product.quantity -= 1;
                productQuantity.textContent = `Quantity: ${product.quantity}`;

                purchaseAmount += 1;
                cartButton.textContent = `Cart (${purchaseAmount})`;

                purchasedItems.set(product.id, (purchasedItems.get(product.id) || 0) + 1);

                if (product.quantity === 0) {
                    productPurchaseButton.disabled = true;
                    productPurchaseButton.textContent = 'Out of Stock';
                }
            }
        });

        const productImage = document.createElement('img');
        productImage.style.width = '150px'; 
        productImage.style.height = '100px';
        productImage.style.backgroundColor = randomColour();

        productItem.appendChild(nameAndPrice);
        productItem.appendChild(productImage);
        productItem.appendChild(productQuantity);
        productItem.appendChild(productPurchaseButton);

        productList.appendChild(productItem);
    });
}

function randomColour() {
    const letters = '0123456789ABCDEF';
    let colour = '#';
    for (let i = 0; i < 6; i++) {
        colour += letters[Math.floor(Math.random() * 16)];
    }
    return colour;
}
