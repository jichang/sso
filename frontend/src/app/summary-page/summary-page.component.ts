import { Component, OnInit } from '@angular/core';
import { User, session } from '../model';

@Component({
  selector: 'summary-page',
  templateUrl: './summary-page.component.html',
  styleUrls: ['./summary-page.component.css']
})
export class SummaryPageComponent implements OnInit {
  user: User;

  constructor() {
    this.user = session.currUser();
  }

  ngOnInit() {
  }

}
