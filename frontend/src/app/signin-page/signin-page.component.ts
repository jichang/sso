import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import * as model from '../model';

@Component({
  selector: 'signin-page',
  templateUrl: './signin-page.component.html',
  styleUrls: ['./signin-page.component.css']
})
export class SigninPageComponent implements OnInit {

  constructor(private router: Router) { }

  ngOnInit() {
  }

  signed({ jwt, user }: { jwt: string, user: model.User }) {
    window.localStorage.setItem('jwt', jwt);
    window.localStorage.setItem('currUser', JSON.stringify(user));

    this.router.navigate(['dashboard']);
  }
}
