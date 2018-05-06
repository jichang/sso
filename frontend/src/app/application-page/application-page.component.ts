import { Component, OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { Application, ApplicationModelService } from '../application-model.service';
import { map } from 'rxjs/operators';

@Component({
  selector: 'application-page',
  templateUrl: './application-page.component.html',
  styleUrls: ['./application-page.component.css']
})
export class ApplicationPageComponent implements OnInit {
  application: Application = null;

  constructor(private route: ActivatedRoute, private applicationModel: ApplicationModelService) { }

  ngOnInit() {
    this.applicationModel.applications.pipe(
      map(applications => applications.find(application => application.id === parseInt(this.route.snapshot.params['id'])))
    ).subscribe(application => {
      this.application = application;
    });

    this.applicationModel.select();
  }
}
