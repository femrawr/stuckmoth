const sidebar = document.querySelector('.sidebar');

const createToggle = (name) => {
    const item = document.createElement('div');
    item.className = 'checkbox';

    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';

    const label = document.createElement('label');
    label.textContent = name;

    item.appendChild(checkbox);
    item.appendChild(label);
    sidebar.appendChild(item);

    return checkbox;
};

createToggle('hide files after encrypt');
createToggle('reset device after encrypt');
createToggle('delete self after encrypt').checked = true;