import http from 'k6/http';
import { check, sleep, group } from 'k6';

// Define target base URL (falls back to local k8s ingress if not overridden by env)
const BASE_URL = __ENV.TARGET_URL || 'http://open-diy.opendiy.vn';

export const options = {
    stages: [
        { duration: '30s', target: 20 },  // Ramp-up to 20 users
        { duration: '1m', target: 20 },   // Stay at 20 users
        { duration: '30s', target: 50 },  // Ramp-up to 50 users (higher load)
        { duration: '1m', target: 50 },   // Stay at 50 users
        { duration: '30s', target: 0 },   // Ramp-down to 0 users
    ],
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% of requests must complete under 500ms
        http_req_failed: ['rate<0.01'],   // Error rate must be less than 1%
    },
};

const PRODUCTS = ['dactyl', 'frosted', 'alice'];

export default function () {
    // 1. Browsing Home Page
    group('Home Page Access', function () {
        const res = http.get(`${BASE_URL}/`);
        check(res, {
            'status is 200': (r) => r.status === 200,
            'body contains open-diy': (r) => r.body.includes('open-diy') || r.body.includes('open_diy') || r.body.includes('navbar'),
        });
        sleep(1 + Math.random() * 2); // Sleep 1 to 3 seconds
    });

    // 2. Accessing Shop Catalog
    group('Shop Catalog Access', function () {
        const res = http.get(`${BASE_URL}/shop`);
        check(res, {
            'status is 200': (r) => r.status === 200,
            'body contains products': (r) => r.body.includes('Catalog') || r.body.includes('Sản phẩm') || r.body.includes('product'),
        });
        sleep(2 + Math.random() * 3); // Sleep 2 to 5 seconds
    });

    // 3. Browsing Random Product Details
    group('Product Details Access', function () {
        const randomProduct = PRODUCTS[Math.floor(Math.random() * PRODUCTS.length)];
        const res = http.get(`${BASE_URL}/product/${randomProduct}`);
        check(res, {
            'status is 200': (r) => r.status === 200,
            'body contains product detail': (r) => r.body.includes('detail') || r.body.includes('specs') || r.body.includes(randomProduct),
        });
        sleep(2 + Math.random() * 4); // Sleep 2 to 6 seconds
    });

    // 4. Loading Static Assets (Simulate browser behavior)
    group('Static Assets Loading', function () {
        // Load the main CSS bundle, JS hooks, and product image in parallel
        const responses = http.batch([
            ['GET', `${BASE_URL}/js/audio.js`, null, { tags: { name: 'audio-js' } }],
            ['GET', `${BASE_URL}/images/dactyl.png`, null, { tags: { name: 'dactyl-image' } }],
        ]);

        check(responses[0], {
            'audio.js status is 200': (r) => r.status === 200 || r.status === 304,
        });
        check(responses[1], {
            'image status is 200': (r) => r.status === 200 || r.status === 304,
        });
        
        sleep(1);
    });

    // 5. Viewing About Page
    group('About Page Access', function () {
        const res = http.get(`${BASE_URL}/about`);
        check(res, {
            'status is 200': (r) => r.status === 200,
            'body contains about info': (r) => r.body.includes('about') || r.body.includes('Giới thiệu'),
        });
        sleep(1 + Math.random() * 2);
    });
}
