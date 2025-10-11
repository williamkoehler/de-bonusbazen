import { Routes } from '@angular/router';
import { HomePageComponent } from './pages/home/home.page.component';
import { BlogPageComponent } from './pages/blog/blog.page.component';
import { AboutPageComponent } from './pages/about/about.page.component';
import { AccountPageComponent } from './pages/account/account.page.component';
import { LoginPageComponent } from './pages/login/login.page.component';
import { RegisterPageComponent } from './pages/register/register.page.component';
import { AhMostBonusPageComponent } from './pages/ah/ah.most-bonus.page/ah.most-bonus.page.component';

export const routes: Routes = [
    {
        path: '',
        redirectTo: 'home',
        pathMatch: 'full'
    },
    {
        path: 'home',
        component: HomePageComponent
    },
    {
        path: 'blog',
        component: BlogPageComponent
    },
    {
        path: 'ah',
        children: [
            {
                path: 'mostBonus',
                component: AhMostBonusPageComponent,
            }
        ]
    },
    {
        path: 'about',
        component: AboutPageComponent
    },

    {
        path: 'login',
        component: LoginPageComponent,
    },
    {
        path: 'register',
        component: RegisterPageComponent,
    },
    {
        path: 'account',
        component: AccountPageComponent,
    },
];
