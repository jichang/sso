import { Component, OnInit, Input } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { Application, ApplicationModelService } from '../application-model.service';

@Component({
  selector: 'application-basic',
  templateUrl: './application-basic.component.html',
  styleUrls: ['./application-basic.component.css']
})
export class ApplicationBasicComponent implements OnInit {
  application: Application = null;

  constructor(private route: ActivatedRoute, private applicationModel: ApplicationModelService) { }

  ngOnInit() {
    this.applicationModel.applications
      .map(applications => applications.find(application => application.id === parseInt(this.route.parent.snapshot.params['id'])))
      .subscribe(application => {
        this.application = application;
      });
  }

}
