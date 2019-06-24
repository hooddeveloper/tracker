import Vue from 'vue'
import App from '@/App.vue'
import router from '@/router'

Vue.config.productionTip = false;

// Vue.use(VueRouter)

router.beforeEach((to, from, next) => {
    document.title = to.meta.title;
    next()
});

const vue = new Vue({
    router,
    render: h => h(App)
});

vue.$mount('#app');
