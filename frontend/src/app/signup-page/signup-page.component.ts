import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import * as model from '../model';

@Component({
  selector: 'signup-page',
  templateUrl: './signup-page.component.html',
  styleUrls: ['./signup-page.component.css']
})
export class SignupPageComponent implements OnInit {

  constructor(private router: Router) { }

  ngOnInit() {
  }

  created({ jwt, user }: { jwt: string, user: model.User }) {
    window.localStorage.setItem('jwt', jwt);
    window.localStorage.setItem('currUser', JSON.stringify(user));

    this.router.navigate(['dashboard']);
  }
}
